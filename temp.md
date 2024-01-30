app.js 

document.addEventListener('DOMContentLoaded', function () {
    const dispenseButton = document.getElementById('dispenseButton');

    dispenseButton.addEventListener('click', async function () {
        try {
            const response = await fetch('http://localhost:3000/dispense', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ container_index: 1 }), // Adjust the payload as needed
            });

            if (response.ok) {
                const result = await response.json();
                alert(result.message);
            } else {
                console.error('Failed to dispense medication:', response.status);
            }
        } catch (error) {
            console.error('An error occurred:', error);
        }
    });
});


index.html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Simple Frontend</title>
</head>
<body>
    <h1>Simple Frontend</h1>
    <button id="dispenseButton">Dispense Medication</button>

    <script src="app.js"></script>
</body>
</html>

index.js

const express = require('express');
const bodyParser = require('body-parser');

const app = express();
const port = 3000;

// Parse JSON requests
app.use(bodyParser.json());

// Define your API endpoint
app.post('/dispense', (req, res) => {
  // Retrieve data from the request body
  const { containerIndex } = req.body;

  // Replace the following with your actual logic
  const responseMessage = `Dispensing medication for container ${containerIndex}`;

  // Send a JSON response
  res.json({ message: responseMessage });
});

// Start the server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
}
