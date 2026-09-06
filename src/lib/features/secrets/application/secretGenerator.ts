export type GeneratorType = "hex256" | "hex128" | "base64" | "token" | "password" | "uuid";

function generateRandomBytes(size: number): Uint8Array {
  const array = new Uint8Array(size);
  crypto.getRandomValues(array);
  return array;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

function generateUuid(): string {
  if (crypto.randomUUID) return crypto.randomUUID();

  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (character) => {
    const randomValue = (crypto.getRandomValues(new Uint8Array(1))[0] % 16) | 0;
    const value = character === "x" ? randomValue : (randomValue & 0x3) | 0x8;
    return value.toString(16);
  });
}

function generateToken(length: number): string {
  const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  const bytes = generateRandomBytes(length);
  let result = "";
  for (let index = 0; index < length; index++) {
    result += characters[bytes[index] % characters.length];
  }
  return result;
}

function generateStrongPassword(length: number): string {
  const lower = "abcdefghijklmnopqrstuvwxyz";
  const upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  const numbers = "0123456789";
  const symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";
  const allCharacters = lower + upper + numbers + symbols;

  const bytes = generateRandomBytes(length);
  let password = "";
  password += lower[bytes[0] % lower.length];
  password += upper[bytes[1] % upper.length];
  password += numbers[bytes[2] % numbers.length];
  password += symbols[bytes[3] % symbols.length];

  for (let index = 4; index < length; index++) {
    password += allCharacters[bytes[index] % allCharacters.length];
  }

  const characters = password.split("");
  for (let index = characters.length - 1; index > 0; index--) {
    const swapIndex = bytes[index % bytes.length] % (index + 1);
    [characters[index], characters[swapIndex]] = [characters[swapIndex], characters[index]];
  }
  return characters.join("");
}

export function generateSecret(type: GeneratorType, passwordLength: number): string {
  switch (type) {
    case "hex256":
      return bytesToHex(generateRandomBytes(32));
    case "hex128":
      return bytesToHex(generateRandomBytes(16));
    case "base64": {
      const bytes = generateRandomBytes(32);
      let binary = "";
      for (const byte of bytes) binary += String.fromCharCode(byte);
      return btoa(binary);
    }
    case "token":
      return generateToken(32);
    case "password":
      return generateStrongPassword(passwordLength);
    case "uuid":
      return generateUuid();
  }
}
