// @vitest-environment happy-dom
import { expect, test } from "vitest";

test("selectionchange is queued and coalesced while the selection itself updates immediately", async () => {
  const text = document.createTextNode("abc");
  document.body.appendChild(text);
  const selection = document.getSelection()!;
  selection.removeAllRanges();
  await new Promise<void>((resolve) => window.setTimeout(resolve, 0));
  let events = 0;
  const onChange = () => { events++; };
  document.addEventListener("selectionchange", onChange);
  try {
    selection.collapse(text, 0);
    selection.collapse(text, 1);
    expect(selection.focusOffset).toBe(1);
    expect(events).toBe(0);
    await new Promise<void>((resolve) => window.setTimeout(resolve, 0));
    expect(events).toBe(1);
  } finally {
    document.removeEventListener("selectionchange", onChange);
    selection.removeAllRanges();
    text.remove();
  }
});
