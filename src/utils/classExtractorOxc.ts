import oxc from "oxc-parser";

export interface ClassMatch {
  classString: string;
  original: string;
  start: number;
  end: number;
  path: string;
}

export function extractClassesWithOxc(code: string, filename: string, tailwindFunctions: string[]): ClassMatch[] {
  const results: ClassMatch[] = [];
  // Track nodes we've already processed to avoid duplicates
  const processedNodes = new Set<string>();

  try {
    const { program, errors } = oxc.parseSync(filename, code);

    if (errors.length > 0) {
      console.warn("Parsing warnings:", errors);
    }

    // biome-ignore lint/suspicious/noExplicitAny: We can improve the type safety later
    function isInTailwindContext(parentStack: any[]): boolean {
      for (let i = parentStack.length - 1; i >= 0; i--) {
        const parent = parentStack[i];

        // Direct className/class attribute
        if (
          parent?.type === "JSXAttribute" &&
          parent.name?.type === "JSXIdentifier" &&
          (parent.name.name === "className" || parent.name.name === "class")
        ) {
          return true;
        }

        // Inside a tailwind function call
        if (parent?.type === "CallExpression" && parent.callee?.type === "Identifier" && tailwindFunctions.includes(parent.callee.name)) {
          return true;
        }
      }
      return false;
    }

    // biome-ignore lint/suspicious/noExplicitAny: We can improve the type safety later
    function visitNode(node: any, parentStack: any[] = [], path = "root"): void {
      if (!node || typeof node !== "object") {
        return;
      }

      // Create a unique identifier for this node based on type, position and path
      const nodeId = `${node.type}:${node.start}:${node.end}:${path}`;

      // Skip if we've already processed this exact node
      if (processedNodes.has(nodeId)) {
        return;
      }
      processedNodes.add(nodeId);

      const inTailwindContext = isInTailwindContext(parentStack);

      // Handle string literals
      if (node.type === "Literal" && typeof node.value === "string") {
        if (inTailwindContext) {
          const value = node.value.trim();
          if (value) {
            results.push({
              classString: value,
              original: node.raw,
              start: node.start,
              end: node.end,
              path,
            });
          }
        }
      }
      // Handle template literals
      else if (node.type === "TemplateLiteral" && inTailwindContext) {
        // Handle the static parts
        // biome-ignore lint/suspicious/noExplicitAny: We can improve the type safety later
        node.quasis?.forEach((quasi: any, index: number) => {
          const value = quasi.value.cooked.trim();
          if (value) {
            results.push({
              classString: value,
              original: quasi.value.raw,
              start: quasi.start,
              end: quasi.end,
              path: `${path}.quasis[${index}]`,
            });
          }
        });

        // Handle the expressions
        // biome-ignore lint/suspicious/noExplicitAny: We can improve the type safety later
        node.expressions?.forEach((expr: any, index: number) => {
          visitNode(expr, [...parentStack, node], `${path}.expressions[${index}]`);
        });
      }
      // Handle conditional expressions
      else if (node.type === "ConditionalExpression") {
        visitNode(node.consequent, [...parentStack, node], `${path}.consequent`);
        visitNode(node.alternate, [...parentStack, node], `${path}.alternate`);
      }
      // Handle object expressions and their properties
      else if (node.type === "ObjectExpression") {
        // biome-ignore lint/suspicious/noExplicitAny: We can improve the type safety later
        node.properties?.forEach((prop: any, index: number) => {
          visitNode(prop, [...parentStack, node], `${path}.properties[${index}]`);
        });
      }
      // Handle property keys and values
      else if (node.type === "Property") {
        if (node.key?.type === "Literal") {
          visitNode(node.key, [...parentStack, node], `${path}.key`);
        }
        visitNode(node.value, [...parentStack, node], `${path}.value`);
      }

      // Continue traversal for all other nodes
      for (const key in node) {
        if (key === "loc" || key === "range" || key === "parent") {
          continue;
        }

        const value = node[key];
        if (Array.isArray(value)) {
          value.forEach((item, index) => {
            if (item && typeof item === "object") {
              visitNode(item, [...parentStack, node], `${path}.${key}[${index}]`);
            }
          });
        } else if (value && typeof value === "object") {
          visitNode(value, [...parentStack, node], `${path}.${key}`);
        }
      }
    }

    visitNode(program);

    // Sort by position to ensure replacements are done in the right order
    return results.sort((a, b) => b.start - a.start);
  } catch (error) {
    console.error("Failed to parse code:", error);
    return [];
  }
}
