var sidebar = document.getElementById("sidebar-chat");
var username = fetchCookie("username");

var chat_id = 0; // chat currently opened

send_request(`http://127.0.0.1:8000/chat/sidebar/${username}`, (r) => {
    //sidebar.innerHTML = '';

    let data = JSON.parse(r);

    for (let element in data) {
        sidebar.innerHTML += `
        <div class="sidebar-profile${data[element][2] ? ' unread' : ''}" onclick="openChat(${element})">
            <img src="./assets/profile.png">
            <div id="user-info">
                <h1>${data[element][0]}</h1>
                <span>
                    <h2>${data[element][1]}</h2>
                    <h2 id="date">• ${data[element][3]}</h2>
                </span>
            </div>
        </div>
        `
    };
})

// <div class="sidebar-profile">
//     <img src="./assets/profile.png">
//     <div id="user-info">
//         <h1>fafaloveicecream 🍦</h1>
//         <h2>Lorem, ipsum dolor sit</h2>
//     </div>
// </div>

async function send_request(url, func) {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            func(this.responseText);
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


function openChat(id) {
    console.log(`Opening chat ${id}`);
}
