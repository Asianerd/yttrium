let userID = fetchCookie('user_id');

async function debug() {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            var post_data = JSON.parse(this.responseText);
        }
    }

    http.open("GET", `http://127.0.0.1:8000/`, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}

async function fill_chat_data() {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            var chatData = JSON.parse(this.responseText);
            console.log(chatData);
        }
    }

    http.open("GET", `http://127.0.0.1:8000/`, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}

async function send_request(url) {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            return this.responseText;
        }
    }

    http.open("GET", url, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}

function fetchCookie(name) {
    var result = undefined;
    document.cookie.split(';').forEach(element => {
        let x = element.trim().split("=");
        if (x[0] == name) {
            result = x[x.length - 1];
        }
    });
    return result;
}



//document.cookie = `test=${index}; expires=Mon, 01 Jan 2030 00:00:00 UTC; path=/`;

