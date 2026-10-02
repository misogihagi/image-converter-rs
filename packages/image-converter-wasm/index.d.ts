/**
 * Converts image bytes to the specified format.
 *
 * @param inputBytes The source image bytes (Uint8Array)
 * @param format Output format ("png", "jpg", "jpeg", "webp", "bmp", "tiff")
 * @returns Converted image bytes (Uint8Array)
 */
export function convert(inputBytes: Uint8Array, format: string): Uint8Array;

/**
 * Automatically detects the format of the input image bytes.
 *
 * @param inputBytes The source image bytes (Uint8Array)
 * @returns Format string ("heic", "jpeg", "png", "webp", "bmp", "tiff", "gif")
 */
export function detect_format(inputBytes: Uint8Array): string;

/**
 * Checks whether the image bytes are HEIC/HEIF format.
 *
 * @param inputBytes The image bytes (Uint8Array)
 * @returns true if HEIC/HEIF, otherwise false
 */
export function is_heic(inputBytes: Uint8Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;
export type SyncInitInput = BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly convert: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly detect_format: (a: number, b: number) => [number, number, number, number];
  readonly is_heic: (a: number, b: number) => number;
}

/**
 * (Browser only) Initialize WebAssembly module.
 * Not required in Node.js environment as it is automatically loaded.
 */
export default function init(moduleOrPath?: InitInput | Promise<InitInput>): Promise<InitOutput>;
export function initSync(module: SyncInitInput | { module: SyncInitInput }): InitOutput;
