// Browser-safe port of dotenv's parse(); the dotenv package imports Node-only modules.
const LINE =
  /(?:^|^)\s*(?:export\s+)?([\w.-]+)(?:\s*=\s*?|:\s+?)(\s*'(?:\\'|[^'])*'|\s*"(?:\\"|[^"])*"|\s*`(?:\\`|[^`])*`|[^#\r\n]+)?\s*(?:#.*)?(?:$|$)/gm;

export function parse(source: string): Record<string, string> {
  const result: Record<string, string> = {};
  const text = source.replace(/\r\n?/gm, "\n");
  const pattern = new RegExp(LINE.source, LINE.flags);

  let match: RegExpExecArray | null;
  while ((match = pattern.exec(text)) !== null) {
    let value = (match[2] ?? "").trim();
    const quote = value[0];
    value = value.replace(/^(['"`])([\s\S]*)\1$/gm, "$2");
    if (quote === '"') value = value.replace(/\\n/g, "\n").replace(/\\r/g, "\r");
    result[match[1]] = value;
  }

  return result;
}
