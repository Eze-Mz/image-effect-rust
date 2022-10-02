async function init() {
  //varaible to import a module
  let rustApp = null;

  //try to import webassembly module
  try {
    rustApp = await import("../pkg");
  } catch (e) {
    console.error(e);
    return;
  }

  console.log(rustApp);

  //get the input tag an instance the FileReader browser object
  const input = document.getElementById("upload");
  const reader = new FileReader();

  //transform the file loaded into base64 then transffer it to rust
  reader.onloadend = () => {
    let base64 = reader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );

    let img_data_url = rustApp.grayscale(base64);
    document.getElementById("new-img").setAttribute("src", img_data_url);
  };

  input.addEventListener("change", (e) => {
    reader.readAsDataURL(input.files[0]);
    console.log(reader.result);
  });
}

init();
