import { join, dirname, fromFileUrl } from "https://deno.land/std@0.103.0/path/mod.ts";

import { cheerio } from "https://deno.land/x/cheerio@1.0.4/mod.ts";
import icons from "https://cdn.skypack.dev/simple-icons/index.js?dts";

type SimpleIcon = { slug: string; svg: string };

const svgSprite = cheerio('<svg xmlns="http://www.w3.org/2000/svg" style="display: none;"></svg>');

for (const { slug, svg } of Object.values(icons) as SimpleIcon[]) {
  const $ = cheerio.load(svg);

  const vbox = $("svg").attr("viewBox")!;

  const path = cheerio.html($("svg > path"));

  cheerio("<symbol></symbol>").attr("id", slug).attr("viewBox", vbox).append(path).appendTo(svgSprite);
}

const __dirname = dirname(fromFileUrl(import.meta.url));

const encoder = new TextEncoder();
const svg = encoder.encode(
  `<?xml version="1.0" encoding="UTF-8"?>
${cheerio.html(svgSprite, { xmlMode: true })}`
);

Deno.writeFileSync(join(__dirname, "../badgeland/build_scripts/icons/simple-icons.svg"), svg);
