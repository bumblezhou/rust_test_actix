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

function init_upload_form() {
    document.getElementById('upload_form').addEventListener('submit', function(event) {
        // Prevent the default form submission behavior
        event.preventDefault();
    
        // Optionally, you can perform additional actions here, such as form validation
    
        // Finally, you can submit the form data via AJAX or perform other actions as needed
        var formData = new FormData(this);
    
        fetch(this.action, {
            method: this.method,
            body: formData
        })
        .then(response => {
            if (response.ok) {
                console.log('Form submitted successfully');
                document.getElementById("upload_result").innerText = 'Form submitted successfully';
                document.getElementsByName('file')[0].value = '';
            } else {
                console.error('Failed to submit form');
            }
        })
        .catch(error => {
            console.error('Failed to submit form:', error);
        });
    });
}