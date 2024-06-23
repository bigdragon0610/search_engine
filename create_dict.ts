import $ from "https://deno.land/x/dax@0.39.2/mod.ts";

const result = await $`ls ../blog/articles`.text();

for (const slug of result.split("\n")) {
  const md = await Deno.readTextFile(`../blog/articles/${slug}/index.md`);
  await $`create_dict ./dict.json ${slug} ${md}`;
}
