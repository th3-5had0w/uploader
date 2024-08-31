document.querySelector("#upload-challenge").addEventListener("click", async (e) => {
    const data = new FormData(document.querySelector(".challenge-upload-form"));
    console.log(document.querySelector(".challenge-upload-form"));

    e.preventDefault();

    let result = await fetch("/api/upload", {
        method: "POST",
        body: data,
    });

    result = await result.json();

    if (!result.is_error) {
        alert(result.message);
        location.reload();
    } else {
        alert(result.message);
    }
})