

function add_item() {

    var item_nameInput = document.getElementById("item_name");
    var item_quantityInput = document.getElementById("item_quantity");

    var item_name = item_nameInput.value;
    var item_quantity = item_quantityInput.value;

    // create new elements
    var shopping_list = document.getElementById("shopping_list");
    var list_form = document.createElement("form");
    var item_name = document.createElement("input");
    var item_quant = document.createElement("input");

    // dup values
    item_name.type = "text";
    item_name.name = "item_name";
    item_name.placeholder = "Item Name";
    item_name.value = item_nameInput.value;

    item_quant.type = "text";
    item_quant.name = "item_quantity";
    item_quant.placeholder = "Item Quantity";
    item_quant.value = item_quantityInput.value;

    // append to form
    list_form.className = "item";
    list_form.appendChild(item_name);
    list_form.appendChild(item_quant);

    var delete_button = document.createElement("button");
    delete_button.textContent = "Delete";
    delete_button.onclick = function() {
        list_form.remove();
        var confirm_button = document.getElementById("confirm_button");
        if (get_items().length > 0) {
            confirm_button.disabled = false;
        } else {
            confirm_button.disabled = true;
        }
    };

    list_form.appendChild(delete_button);
    shopping_list.appendChild(list_form);

    item_nameInput.value = "";
    item_quantityInput.value = "";

}

function get_items() {
    var items = []; 
    
    const list_items = document.querySelectorAll("form.item");
    for (let i = 0; i < list_items.length; ++i) {
        var new_item = [];
        const fields = list_items[i].getElementsByTagName("input");
        for (let j = 0; j < fields.length; ++j) {
            if (fields[j].type === "text" || fields[j].type === "number") {
                new_item.push(fields[j].value);
            }
        }
        items.push(new_item);
    }
    return items;
}

function confirm_list() {
    const items = get_items();
    const title = document.getElementById("list-title").value; 

    console.log(items);

    // send POST request
    const json_data = JSON.stringify({title: title, items: items }); 
    console.log("JSON Data: ", json_data);

    const url = 'http://127.0.0.1:4040/';
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: json_data
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Not ok response');
        }
        return response.text(); 
    })
    .then(data => {
        console.log(data); 
    })
    .catch(error => {
        console.error('error in fetch op:', error);
    });
}

