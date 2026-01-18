export class ProductView {
  #buffer;
  #offset;

  constructor(buffer, offset = 0) {
    this.#buffer = buffer;
    this.#offset = offset;
  }

  #getNameLength() {
    return this.#buffer[this.#offset];
  }

  get #bufferOffset() {
    return this.#offset + this.#buffer.byteOffset;
  }

  get #baseOffset() {
    return  this.#bufferOffset;
  }

  get #skuOffset() {
    return this.#baseOffset + 1;
  }

  get #priceOffset() {
    return align(this.#skuOffset + this.#getNameLength(), Float64Array.BYTES_PER_ELEMENT);
  }

  get #quantityOffset() {
    return align(this.#priceOffset + Float64Array.BYTES_PER_ELEMENT, Uint32Array.BYTES_PER_ELEMENT);
  }

  get #inStockOffset() {
    return this.#quantityOffset + Uint32Array.BYTES_PER_ELEMENT;
  }

  get sku() {
    const nameLength = this.#getNameLength();
    return new TextDecoder().decode(this.#buffer.slice(this.#skuOffset, this.#skuOffset + nameLength))
  }

  get price() {
    return new DataView(this.#buffer.buffer, this.#priceOffset, Float64Array.BYTES_PER_ELEMENT).getFloat64(0, true);
  }

  get quantity() {
    return new DataView(this.#buffer.buffer, this.#quantityOffset, Uint32Array.BYTES_PER_ELEMENT).getUint32(0, true);
  }

  get inStock() {
    return this.#buffer[this.#inStockOffset] === 1;
  }

  get length() {
    return align(this.#inStockOffset + 1 - this.#offset, 8);
  }
}

function align(offset, alignment) {
  return (offset + alignment - 1) & ~(alignment - 1);
}