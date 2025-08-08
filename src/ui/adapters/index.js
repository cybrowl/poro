// adapters/index.js

// 'marked' for markdown to HTML, 'katex' for math rendering
const { marked } = require("marked");
const katex = require("katex");

// Helper function to extract embedded JSON tools from content
function extractJsonTools(content) {
  const toolRegex = /\{.*?\}/gs; // Simple regex to find JSON-like objects; adjust for specificity if needed
  const tools = [];
  let match;
  while ((match = toolRegex.exec(content)) !== null) {
    try {
      const parsed = JSON.parse(match[0]);
      if (
        parsed &&
        typeof parsed === "object" &&
        (parsed.tool || parsed.function)
      ) {
        // Check if it's a tool call
        tools.push(parsed);
      }
    } catch (e) {
      // Ignore invalid JSON
    }
  }
  return tools;
}

// Helper function to remove extracted tools from the markdown content
function stripToolsFromMarkdown(content) {
  return content.replace(/\{.*?\}/gs, "").trim(); // Remove JSON blocks and trim whitespace
}

// Helper function to render LaTeX math in markdown using KaTeX
function renderMathInMarkdown(content) {
  // Find inline math: $...$ and display math: $$...$$
  content = content.replace(/\$\$([\s\S]*?)\$\$/g, (match, math) => {
    try {
      return katex.renderToString(math.trim(), {
        displayMode: true,
        throwOnError: false,
      });
    } catch (e) {
      return match; // Fallback to original if error
    }
  });
  content = content.replace(/\$([\s\S]*?)\$/g, (match, math) => {
    try {
      return katex.renderToString(math.trim(), {
        displayMode: false,
        throwOnError: false,
      });
    } catch (e) {
      return match;
    }
  });
  return content;
}

// Handles markdown parsing, tool extraction, math rendering, and extras
// Assumes rawJson is in OpenAI chat completion format: { choices: [{ message: { content, tool_calls? } }] }
export function adaptResponse(model, rawJson) {
  if (
    !rawJson ||
    !rawJson.choices ||
    !rawJson.choices[0] ||
    !rawJson.choices[0].message
  ) {
    throw new Error("Invalid response format");
  }

  let message = rawJson.choices[0].message;
  let content = message.content || "";
  let tools = [];
  let extras = {};

  // Handle OpenAI-specific tool calls if present (standard for GPT models)
  if (message.tool_calls && Array.isArray(message.tool_calls)) {
    tools = message.tool_calls.map((call) => ({
      name: call.function.name,
      arguments: call.function.arguments
        ? JSON.parse(call.function.arguments)
        : {},
    }));
    extras.isAgentic = true; // Flag for UI to show interactive tools
  }

  // Model-specific pre-processing
  if (model === "kimi-k2" || model === "glm-4.5") {
    // Extract embedded tools if not using standard tool_calls
    tools = [...tools, ...extractJsonTools(content)];
    content = stripToolsFromMarkdown(content);
    extras.isAgentic = true;
  } else if (model === "deepseek-r1") {
    content = renderMathInMarkdown(content);
    extras.hasMath = true; // Optional flag for UI styling
  }

  // Default: Parse markdown to HTML (safe for UI rendering)
  const htmlContent = marked.parse(content);

  return { htmlContent, tools, extras };
}
