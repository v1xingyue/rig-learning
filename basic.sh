curl https://api.red-pill.ai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-" \
  -d '{
  "model": "gpt-4o",
  "messages": [
    {
      "role": "user",
      "content": "Hello world!"
    }
  ],
  "temperature": 1
}'