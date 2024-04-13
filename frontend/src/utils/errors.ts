export const formatErrorMessage = (e: unknown): string => {
  // @ts-expect-error TS18046
  const code = e.code ?? "UnknownError";

  // @ts-expect-error TS18046
  const message = e.message ?? "Oops, unknown error.";

  if (code === "BadAuthToken") {
    return "Bad authentication token, please clear site data and try again.";
  }

  console.debug(`Error code=${code} message=${message}`);

  return message;
};
