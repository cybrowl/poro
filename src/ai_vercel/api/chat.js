export default async function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }
  const { prompt } = req.body;
  if (!prompt) {
    return res.status(400).json({ error: "Prompt required" });
  }
  const apiKey = process.env.HYPERBOLIC_API_KEY;
  if (!apiKey) {
    return res.status(500).json({ error: "API key not configured" });
  }
  try {
    const response = await fetch(
      "https://api.hyperbolic.xyz/v1/chat/completions",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${apiKey}`,
        },
        body: JSON.stringify({
          messages: [{ role: "user", content: prompt }],
          model: "openai/gpt-oss-120b",
          max_tokens: 512,
          temperature: 0.7,
          top_p: 0.8,
          stream: false,
        }),
      }
    );
    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }
    const data = await response.json();
    const content = data.choices[0]?.message?.content || "No content";
    res.status(200).json({ content });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
}
