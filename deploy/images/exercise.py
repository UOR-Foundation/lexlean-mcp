#!/usr/bin/env python3
"""The gateway of the images, reached as a client reaches it (IM-03, IM-05).

It runs in a container that shares the network namespace of the gateway, because the model declares
the gateway's address as 127.0.0.1 (IMAGES.md). Standard library only, and its one output is what a
command is asked to print: a failure is a message and exit status 1.
"""

import http.client
import json
import sys
import time

PROTOCOL = "2026-07-28"
PROJECT = {"project": "dir", "files": []}
ENGINE_TOOLS = ["lexlean_check", "lexlean_snapshot", "lexlean_build"]
next_id = 100


def fail(message):
    print(message, file=sys.stderr)
    sys.exit(1)


def meta(tasks):
    capabilities = {"extensions": {"io.modelcontextprotocol/tasks": {}}} if tasks else {}
    return {
        "io.modelcontextprotocol/protocolVersion": PROTOCOL,
        "io.modelcontextprotocol/clientInfo": {"name": "images-exercise", "version": "1.0.0"},
        "io.modelcontextprotocol/clientCapabilities": capabilities,
    }


def message_of(body):
    """The answer of a response: JSON, or the last event of a stream that carries an id."""
    text = body.decode("utf-8")
    if text.lstrip().startswith("{"):
        return json.loads(text)
    found = None
    for line in text.splitlines():
        if line.startswith("data:"):
            event = json.loads(line[5:])
            if "id" in event:
                found = event
    if found is None:
        fail(f"no answer in the response: {text[:400]}")
    return found


def rpc(method, name, params):
    answer = exchange(method, name, params)
    if "result" not in answer:
        fail(f"{method}: {json.dumps(answer)[:600]}")
    return answer["result"]


def exchange(method, name, params):
    global next_id
    next_id += 1
    body = json.dumps({"jsonrpc": "2.0", "id": next_id, "method": method, "params": params})
    headers = {
        "Host": "127.0.0.1",
        "Content-Type": "application/json",
        "Accept": "application/json, text/event-stream",
        "MCP-Protocol-Version": PROTOCOL,
        "Mcp-Method": method,
    }
    if name is not None:
        headers["Mcp-Name"] = name
    connection = http.client.HTTPConnection("127.0.0.1", 5395, timeout=120)
    connection.request("POST", "/mcp", body, headers)
    response = connection.getresponse()
    answer = message_of(response.read())
    connection.close()
    return answer


def call(tool, arguments, tasks=False):
    return rpc("tools/call", tool, {"name": tool, "arguments": arguments, "_meta": meta(tasks)})


def task(method, handle):
    return rpc(method, handle, {"taskId": handle, "_meta": meta(True)})


def wait_ready():
    until = time.monotonic() + 60
    while True:
        try:
            tools = rpc("tools/list", None, {"_meta": meta(False)})["tools"]
            break
        except OSError as error:
            if time.monotonic() > until:
                fail(f"the gateway does not answer: {error}")
            time.sleep(0.5)
    if "lexlean_verify" not in [tool["name"] for tool in tools]:
        fail("tools/list has no lexlean_verify")


def engine():
    for tool in ENGINE_TOOLS:
        result = call(tool, PROJECT)
        if result.get("isError"):
            fail(f"{tool}: {json.dumps(result)[:600]}")


def start():
    result = call("lexlean_verify", PROJECT, tasks=True)
    if "taskId" not in result:
        fail(f"no task: {json.dumps(result)[:600]}")
    print(result["taskId"])


def ended(handle, seconds=900):
    until = time.monotonic() + seconds
    while True:
        result = task("tasks/get", handle)
        if result["status"] != "working":
            return result
        if time.monotonic() > until:
            fail(f"the task {handle} did not end in {seconds} s")
        time.sleep(0.5)


def verified():
    result = call("lexlean_verify", PROJECT, tasks=True)
    done = ended(result["taskId"])
    if done["status"] != "completed":
        fail(f"the task ended {json.dumps(done)[:800]}")
    outcome = done["result"]
    if outcome["structuredContent"]["state"] != "verified":
        fail(f"the verification: {json.dumps(outcome)[:800]}")
    record = json.loads(outcome["content"][0]["text"])
    found = call("lexlean_attestation", {"by": "id", "value": record["attestation_id"]})
    if found.get("isError"):
        fail(f"the record of {record['attestation_id']}: {json.dumps(found)[:600]}")


def cancelled():
    """A cancel that ended a Lean child, told by the message of the task (AC-12).

    A verification of this project takes moments and a cancel between two children has nothing to
    end, so each attempt waits a little longer before it cancels, until one ends a child.
    """
    seen = []
    for delay in (0.1, 0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.5):
        handle = call("lexlean_verify", PROJECT, tasks=True)["taskId"]
        time.sleep(delay)
        answer = exchange("tasks/cancel", handle, {"taskId": handle, "_meta": meta(True)})
        done = ended(handle, 120)
        if "result" not in answer or done["status"] != "cancelled":
            seen.append(done["status"])
            continue
        message = done.get("statusMessage", "")
        if not message.endswith(" failed: 0"):
            fail(f"a child group failed to end: {message}")
        if message != "child groups ended: 0 failed: 0":
            return
        seen.append(message)
    fail(f"no cancel ended a child group: {seen}")


def refused(tool, failure, tasks):
    result = call(tool, PROJECT, tasks=tasks)
    found = result.get("structuredContent")
    if not result.get("isError") or found != {"failure": failure}:
        fail(f"{tool}: expected the failure {failure}, got {json.dumps(result)[:600]}")


def settled(handle):
    """How a task ended: `verified`, or the failure its result names (VP-05)."""
    done = ended(handle, 180)
    outcome = done.get("result", {})
    state = outcome.get("structuredContent", {}).get("state")
    if done["status"] != "completed" or state not in ("verified", "failed"):
        fail(f"the task ended {json.dumps(done)[:700]}")
    if state == "verified":
        print("verified")
    else:
        print(json.loads(outcome["content"][0]["text"])["failure"])


COMMANDS = {
    "ready": wait_ready,
    "engine": engine,
    "start": start,
    "verified": verified,
    "cancelled": cancelled,
    "settled": settled,
    "engine-refused": lambda: refused("lexlean_check", "engine-unavailable", False),
    "verify-refused": lambda: refused("lexlean_verify", "verifier-unavailable", True),
}

if __name__ == "__main__":
    COMMANDS[sys.argv[1]](*sys.argv[2:])
