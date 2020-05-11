function checkAll(value) {
  let collection = document.getElementsByTagName("input");

  for (let i=0; i < collection.length; i++) {
    let input = collection.item(i);

    if (input.value == value && input.name.startsWith("q")) {
      input.checked = true;
    }
  }
}
