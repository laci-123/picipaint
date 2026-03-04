import { buttonText } from "./something.ts";
import { test, expect } from "vitest";

test("string interpolation works", () => {
  expect(buttonText(123)).toBe("pressed 123 times");
})
