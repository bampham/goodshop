class ShoppingList {
    constructor(title, items) {
        this.title = title;
        this.items = items;
    }
}

class GetRequest {
    constructor(shopping_lists) {
        this.shopping_lists = shopping_lists;
    }
}

function display_lists(getRequest) {
    const shopping_lists = document.getElementById("lists");
    shopping_lists.innerHTML = "";

    // create elemtns
    getRequest.shopping_lists.forEach(list => {
        const list_container = document.createElement("div");
        list_container.classList.add("shopping-list");

        const title_element = document.createElement("h2");
        title_element.textContent = list.title;

        const item_list = document.createElement("ul");
        list.items.forEach(item => {
            const list_item = document.createElement("li");
            list_item.textContent = `${item[0]} - Q: ${item[1]}`;
            item_list.appendChild(list_item);
        });

        const edit_button = document.createElement("button");
        edit_button.textContent = "Edit";
        edit_button.addEventListener("click", () => {
            // todo: edit function
            console.log("edit button clicked for shopping list:", list.title);
        });

        const delete_button = document.createElement("button");
        delete_button.textContent = "Delete";
        delete_button.addEventListener("click", () => {
            const confirmed = confirm(`Are you sure to delete the list "${list.title}"?`);
            if (confirmed) {
                delete_list(list.title);
            }
        });

        list_container.appendChild(title_element);
        list_container.appendChild(item_list);
        list_container.appendChild(edit_button); 
        list_container.appendChild(delete_button); 
        shopping_lists.appendChild(list_container);
    });
}

function delete_list(list_title) {
    fetch("http://127.0.0.1:4040", {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ title: list_title, items: [[]] })
    })
    .then(response => {
        if (!response.ok) {
            throw new Error("failed delete");
        }

        location.reload();
    })
    .catch(err => {
        console.error("error deleting: ", err);
    });
}

function fetch_lists() {
    fetch("http://127.0.0.1:4040", {
        method: "GET"
    })
    .then(response => {
        if (!response.ok) {
            throw new Error("Not ok response");
        }
        return response.json();
    })
    .then(data => {
        const getRequest = new GetRequest(data.shopping_lists.map(list => new ShoppingList(list.title, list.items)));
        display_lists(getRequest);
    })
    .catch(err => {
        console.error("Error in fetch op: ", err);
    });
}

fetch_lists();

