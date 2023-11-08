let socket = new WebSocket("ws://localhost:3001");
let outputElement = document.getElementById("output")
let inputElement = document.getElementById("input")
let sendButton = document.getElementById("send")

// Connection opened
socket.addEventListener('open', function (event) {
    outputElement.innerHTML += "Connection established, you may start sending JSON messages<br/>";
});

// Listen for messages
socket.addEventListener('message', function (event) {
    let data = JSON.parse(event.data);
    outputElement.innerHTML += `Message from server: ${data.result}<br/>`;
    outputElement.scrollTop = outputElement.scrollHeight; // scroll to bottom
});

sendButton.addEventListener('click', function(event) {
    let json = inputElement.value.trim();
    if(json){
        try{
            JSON.parse(json); //check if it's valid JSON
            socket.send(json);
        }catch(e){
            outputElement.innerHTML += "Invalid JSON<br/>";
        }

    }
    inputElement.value = '';
});