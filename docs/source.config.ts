import { defineDocs, defineConfig } from "fumadocs-mdx/config";
import { replaceSource } from "fumadocs-core/mdx-plugins/stringifier";

type MarkdownNode = Parameters<typeof replaceSource>[1];
type MarkdownFile = Parameters<typeof replaceSource>[0];

// Preserve component titles and links in processed Markdown using Fumadocs 16's
// authored-source replacement API. Container contents keep their own edits.
function remarkComponentMarkdown() {
  const containers = new Set(["Tabs", "Tab", "Cards", "Card", "Steps", "Step", "Accordions", "Accordion", "Callout"]);
  return (tree: MarkdownNode, file: MarkdownFile) => {
    function visit(node: MarkdownNode) {
      if ("children" in node) {
        for (const child of node.children) visit(child);
      }
      if (node.type !== "mdxJsxFlowElement" || !containers.has(node.name ?? "")) return;
      const attribute = (name: string) => {
        const entry = node.attributes.find((item) => item.type === "mdxJsxAttribute" && item.name === name);
        return entry && "value" in entry && typeof entry.value === "string" ? entry.value : undefined;
      };
      const title = attribute("title") ?? attribute("value");
      const href = attribute("href");
      const heading = title ? `### ${href ? `[${title}](${href})` : title}\n\n` : "";
      replaceSource(file, node, (state) => heading + state.inner(node));
    }
    visit(tree);
  };
}

export const docs = defineDocs({
  dir: "content/docs",
  docs: {
    postprocess: { includeProcessedMarkdown: { headingIds: false } },
  },
});

export default defineConfig({
  mdxOptions: { remarkPlugins: [remarkComponentMarkdown] },
});
