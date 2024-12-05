#!/usr/bin/env -S deno run --allow-net

import { DOMParser } from "jsr:@b-fuze/deno-dom";

const URL = "https://www.nesdev.org/obelisk-6502-guide/reference.html";

type Instruction = {
  name: string;
  addressingMode: string;
  opcode: string;
  bytes: string;
  cycles: string;
};

const fetchSource = async () => {
  const res = await fetch(URL);
  return res.text();
};

const parse = async () => {
  const source = await fetchSource();
  const parser = new DOMParser();
  const doc = parser.parseFromString(source, "text/html");

  const tables = doc.querySelectorAll("table[border='1']");
  const names = doc.querySelectorAll("h3 > a[name]");

  const instructions: Instruction[] = [];

  let cnt = 0;
  for (const table of tables) {
    const rows = table.querySelectorAll("tr");

    let flag = false;
    for (const row of rows) {
      const cols = row.querySelectorAll("td");
      if (cols.length !== 4) {
        break;
      }

      if (cols[0].textContent.trim() === "Addressing Mode") {
        continue;
      }

      const name = names[cnt].getAttribute("name");
      if (name === null) {
        break;
      }

      const instruction = {
        name: name ?? "",
        addressingMode: cols[0].textContent.trim(),
        opcode: cols[1].textContent.trim(),
        bytes: cols[2].textContent.trim(),
        cycles: cols[3].textContent.trim(),
      } satisfies Instruction;

      instructions.push(instruction);

      flag = true;
    }

    if (flag) {
      cnt++;
    }
  }

  return instructions;
};

const convertAddressingMode = (addressingMode: string) => {
  const mode = addressingMode
    .split("\n")
    .map((x) => x.trim())
    .join(" ");

  switch (mode) {
    case "Implied":
      return "AddressingMode::Implied";
    case "Accumulator":
      return "AddressingMode::Accumulator";
    case "Immediate":
      return "AddressingMode::Immediate";
    case "Zero Page":
      return "AddressingMode::ZeroPage";
    case "Zero Page,X":
      return "AddressingMode::ZeroPageX";
    case "Zero Page,Y":
      return "AddressingMode::ZeroPageY";
    case "Relative":
      return "AddressingMode::Relative";
    case "Absolute":
      return "AddressingMode::Absolute";
    case "Absolute,X":
      return "AddressingMode::AbsoluteX";
    case "Absolute,Y":
      return "AddressingMode::AbsoluteY";
    case "(Indirect,X)":
      return "AddressingMode::IndirectX";
    case "(Indirect),Y":
      return "AddressingMode::IndirectY";
    case "Indirect":
      return "AddressingMode::Indirect";
    default:
      throw new Error(`Unknown addressing mode: ${addressingMode}`);
  }
};

const convertCycles = (cycles: string) => {
  const match = cycles.trim().match(/^(\d+)/); // 文字列の先頭にある数字を取得

  if (match === null) {
    throw new Error(`Invalid cycles: ${cycles}`);
  }

  return Number(match[1]);
};

const generate = (instructions: Instruction[]) => {
  for (const instruction of instructions) {
    const opcode = instruction.opcode.replace(/\$/g, "0x");
    const bytes = Number.parseInt(instruction.bytes, 10);
    const cycles = convertCycles(instruction.cycles);
    const addressingMode = convertAddressingMode(instruction.addressingMode);

    const source = `OpCode::new(${opcode}, "${instruction.name}", ${bytes}, ${cycles}, ${addressingMode})`;

    console.log(`${source}`);
  }
};

const main = async () => {
  const instructions = await parse();
  generate(instructions);
};

main();
