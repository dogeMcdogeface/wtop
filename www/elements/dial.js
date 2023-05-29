import { Updatable } from '/elements/updatable.js';

class Dial extends Updatable {
    constructor() {
        super();
    }
}
customElements.define('dial', Dial);