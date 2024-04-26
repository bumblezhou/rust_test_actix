function page_load() {
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