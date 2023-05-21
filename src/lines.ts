import fs from "fs";

fs.readFileSync("lines", "utf8")
  .toString()
  .split("\n")
  .filter((_x, i) => i % 2 === 0)
  .forEach((line) => console.log(line));
