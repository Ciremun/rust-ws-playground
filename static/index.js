function request_random_number() {
    window.ws.send('rand');
}

window.onload = () => {
    window.ws = new WebSocket(`ws://${location.host}/ws`);

    ws.onopen = () => {
        console.log("JavaScript: Socket Connected!");
    };

    ws.onmessage = (event) => {
        console.log(event);
        let [command, value] = event.data.split(' ');
        if (command === 'rand') {
            if (+value >= 75)
                document.body.classList.add('inverse');
            else
                document.body.classList.remove('inverse');
            window.random_number.innerText = value;
        }
    };
};
