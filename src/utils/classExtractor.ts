export interface ClassMatch {
  original: string;
  classString: string;
}

/**
 * Extracts Tailwind class strings from both className attributes and configured function calls
 */
export function extractClasses(input: string, tailwindFunctions: string[]): ClassMatch[] {
  const results: ClassMatch[] = [];

  // Regular class/className/classList attributes
  const classRegex = /class(?:Name|List)?=["'`]([^"'`]*?)["'`]/g;
  let match: RegExpExecArray | null;
  while (true) {
    match = classRegex.exec(input);
    if (match === null) {
      break;
    }

    const trimmed = match[1].trim();
    if (trimmed) {
      // Only add non-empty strings
      results.push({
        original: match[0],
        classString: trimmed,
      });
    }
  }

  // Handle configured function calls
  if (tailwindFunctions.length > 0) {
    const functionPattern = new RegExp(`(?:${tailwindFunctions.join("|")})\\s*\\([^)]*?["']\([^"']*\)["']|["']([^"']+)["']`, "g");
    const stringPattern = /["'`]([^"'`]+)["'`]/g;

    let functionMatch: RegExpExecArray | null;
    while (true) {
      functionMatch = functionPattern.exec(input);
      if (functionMatch === null) {
        break;
      }

      let stringMatch: RegExpExecArray | null;
      while (true) {
        stringMatch = stringPattern.exec(functionMatch[0]);
        if (stringMatch === null) {
          break;
        }

        const trimmed = stringMatch[1].trim();
        // Skip empty strings and strings that don't look like class names
        if (trimmed && /^[a-zA-Z0-9:_/-]/g.test(trimmed)) {
          results.push({
            original: stringMatch[0],
            classString: trimmed,
          });
        }
      }
    }
  }

  return results;
}
