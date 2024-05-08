function init_login() {
    document.getElementById('loginForm').addEventListener('submit', function(event) {
        event.preventDefault(); // Prevent default form submission
        // Get form data
        const formData = new FormData(this);
        // Convert FormData to URL-encoded string
        const urlEncodedFormData = new URLSearchParams(formData).toString();
        // Send form data to server using AJAX
        fetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
            body: urlEncodedFormData
        })
        .then(response => response.text())
        .then(data => {
            document.getElementById('message').innerText = data;
            if (data == "Authenticated") {
                window.location.href = "/";
            }
        })
        .catch(error => {
            console.error('Error:', error);
        });
    });
}

function logout() {   
    document.cookie = 'username=; Max-Age=-99999999;';
    window.location.href = "/";
}

let socket = undefined;
function init_websocket() {
    socket = new WebSocket("ws://localhost:8080/ws/");
    socket.onopen = function(e) {
        console.log("WebSocket connection established.");
    };

    socket.onmessage = function(event) {
        console.log("Message received:", event.data);
        // Handle the incoming message, e.g., display it on the webpage
        document.getElementById("msg_list").innerHTML += '<span>' + event.data + '</span><br />';
    };

    socket.onclose = function(event) {
        console.log("WebSocket connection closed.");
    };
}

function send_websocket_msg () {
    if (socket) {
        let message = document.getElementById("text").value;
        if (socket.readyState === WebSocket.OPEN) {
            socket.send(message);
            console.log("Message sent:", message);
        } else {
            console.error("WebSocket connection is not open.");
        }
    }
}