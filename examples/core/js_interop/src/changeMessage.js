// Changes some text in a specific HTML element
// If possible, try to never use JS, `web-sys` will hopefully cover everything you want to do
// But, sometimes, it's unavoidable, so Perseus supports interop easily
export function changeMessage() {
  document.getElementById("message").innerHTML = "Message from JS!";
}
