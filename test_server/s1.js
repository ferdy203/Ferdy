const http = require("http");

// Create the HTTP server
const server = http.createServer((req, res) => {
  console.log("Request Received");
  // Set the response header
  res.writeHead(200, { "Content-Type": "text/plain" });

  // Send the response body
  res.end("Hello, World!\n");
});

// Start the server on port 8080
const PORT = 8080;
server.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
