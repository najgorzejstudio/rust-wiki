const input = document.getElementById("searchbar");
const auto_list = document.getElementById("auto_list");

input.addEventListener("input", async (e) => {
    const value = e.target.value;

    const res = await fetch(`/api/search?q=${encodeURIComponent(value)}`);
    const data = await res.json();

    auto_list.innerHTML = "";

    for (const [title, link] of data) {
        const line = document.createElement("li");
        line.innerHTML = "<a href='"+link+"'>"+title+"</a>";
        auto_list.appendChild(line);
    }
});

