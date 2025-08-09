export default async function handler(req, res) {
  // Validate method
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  // Validate input
  const { prompt } = req.body || {};
  if (!prompt) {
    return res.status(400).json({ error: "Prompt required" });
  }

  // Validate configuration
  const apiKey = process.env.HYPERBOLIC_API_KEY;
  if (!apiKey) {
    return res.status(500).json({ error: "API key not configured" });
  }

  // Request setup
  const endpoint = "https://api.hyperbolic.xyz/v1/chat/completions";
  const headers = {
    "Content-Type": "application/json",
    Authorization: `Bearer ${apiKey}`,
  };
  const payload = {
    messages: [{ role: "user", content: prompt }],
    model: "openai/gpt-oss-120b",
    max_tokens: 1024,
    temperature: 0.5,
    top_p: 0.95,
    stream: false,
  };

  try {
    const response = await fetch(endpoint, {
      method: "POST",
      headers,
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }

    const data = await response.json();
    const content = data?.choices?.[0]?.message?.content || "No content";

    return res.status(200).json({ content });
  } catch (error) {
    return res.status(500).json({ error: error.message });
  }
}
