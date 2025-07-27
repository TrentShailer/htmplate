import { setHref } from "./lib/redirect.ts";
import { Form } from "./lib/form.ts";
import { Temporal } from "./lib/temporal.ts";

const register = new Form("/register", ["/username", "/displayName"], "register");

register.form.addEventListener("submit", (event) => {
  event.preventDefault();

  register.setLock(true);
  register.clearErrors();
  const values = register.getValues();

  const username = values.get("/username") ?? "";
  const displayName = values.get("/displayName") ?? "";
  console.log(username);
  console.log(displayName);

  setTimeout(function () {
    const problems = [
      { pointer: "/username", detail: "too long" },
      { pointer: "/displayName", detail: "too short" },
      { pointer: "/displayName", detail: "contains illegal character" },
      { pointer: "/authorization", detail: "unauthorized" },
    ];

    register.setInputErrors(problems);
    register.setLock(false);
  }, 1000);
});

const redirect = document.getElementById("redirect");
if (redirect) {
  redirect.addEventListener("click", async () => {
    await setHref("/source");
    console.log("Shouldn't execute");
  });
}
console.log(Temporal.Now);
