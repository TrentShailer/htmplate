export function lockForm(
  inputIds: string[],
  submitId: string,
  formErrorId: string,
): Map<string, string> {
  // Disable submit
  const submitElement = document.getElementById(submitId);
  if (submitElement && submitElement instanceof HTMLButtonElement) {
    submitElement.disabled = true;
  }

  // Disable inputs and collect values
  const values = new Map();
  for (const inputId of inputIds) {
    const element = document.getElementById(inputId);
    if (element && element instanceof HTMLInputElement) {
      element.disabled = true;
      element.ariaInvalid = "false";
      values.set(inputId, element.value);
    }

    // Reset error field
    const errorElement = document.getElementById(inputId + ".error");
    if (errorElement) {
      errorElement.classList.add("hidden");
      errorElement.ariaHidden = "true";
      errorElement.textContent = "!";
    }
  }

  // Reset form error
  const formError = document.getElementById(formErrorId);
  if (formError) {
    formError.classList.add("collapse");
    formError.ariaHidden = "true";
  }

  // Reset form error content
  const formErrorContent = document.getElementById(formErrorId + ".content");
  if (formErrorContent) {
    formErrorContent.textContent = "";
  }

  return values;
}

export function setFormInputErrors(
  inputIdMap: Map<string, string>,
  formErrorId: string,
  problems: { pointer: string | null; detail: string | null }[] | null,
  formErrorPrefix: string,
) {
  if (!problems || problems.length === 0) {
    setFormError(formErrorId, "form is invalid");
    return;
  }

  for (const problem of problems) {
    let errorElement = null;
    let inputElement = null;
    let message = " at least one field is invalid.";

    if (problem.pointer) {
      const inputId = inputIdMap.get(problem.pointer);
      if (inputId) {
        inputElement = document.getElementById(inputId);
        errorElement = document.getElementById(inputId + ".error");
        message = "Invalid value";
      }
    }

    if (problem.detail) {
      message = problem.detail;
    }

    if (!errorElement) {
      setFormError(formErrorId, formErrorPrefix + message);
      continue;
    }

    if (errorElement.textContent !== "!") {
      errorElement.textContent += `\n${message}`;
    } else {
      errorElement.textContent = message;
    }

    errorElement.classList.remove("hidden");
    errorElement.ariaHidden = "false";

    if (inputElement) {
      inputElement.ariaInvalid = "true";
    }
  }
}

export function setFormError(formErrorId: string, error: string) {
  const element = document.getElementById(formErrorId);
  const content = document.getElementById(formErrorId + ".content");

  if (!element || !content) {
    alert(error);
    return;
  }

  element.classList.remove("collapse");
  element.ariaHidden = "false";

  if (content.textContent !== "") {
    content.textContent += "\n";
  }
  content.textContent += error;
}

export function unlockForm(inputIds: string[], submitId: string) {
  const submitElement = document.getElementById(submitId);
  if (submitElement && submitElement instanceof HTMLButtonElement) {
    submitElement.disabled = false;
  }

  for (const inputId of inputIds) {
    const element = document.getElementById(inputId);
    if (element && element instanceof HTMLInputElement) {
      element.disabled = false;
    }
  }
}
