document.getElementById("item-form").addEventListener("submit", function(event) {
    event.preventDefault();
    const item_input = document.getElementById("item");
    const item = item_input.value.trim();
    if (item !== "") {
        add_item(item);
        item_input.value = "";
    }
});

function add_item(item) {
    const item_list = document.getElementById("list");
    const list_item = document.createElement("li");
    list_item.textContent = item;
    item_list.appendChild(list_item);
}

function confirm_list() {
    const items = [];
    const list_items = document.querySelectorAll("#list li");
    list_items.forEach(function(item) {
        items.push(item.textContent);
    });

    const json_data = JSON.stringify({ items: items }); 

    const http = new XMLHttpRequest();
    const url = 'http://127.0.0.1:4040/';

    http.open('POST', url);
    http.setRequestHeader('Content-Type', 'application/json');

    http.onreadystatechange = function() {
        if (http.readyState === 4) {
            if (http.status === 200) {
                console.log(http.responseText);
            } else {
                console.error('Error:', http.status, http.statusText);
            }
        }
    };

    http.send(json_data);
}

