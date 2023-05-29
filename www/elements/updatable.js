export class Updatable extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }
    connectedCallback() {
        this.classList.add('custom');
    }
    update(value) {
        this.textContent = value;
    }
}
