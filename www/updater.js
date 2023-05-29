
window.addEventListener('load', function() {
    setInterval(makeRequest, 400);
    const upd_elements = document.querySelectorAll('[data-value]');

    /******************************************************************************************************************/
    function makeRequest() {
        fetch('/api/authorized')
            .then(response => response.json())
            .then(displayValues)
            .catch(error => console.error('Error:', error));
    }
    /******************************************************************************************************************/
    function displayValues(response) {
        upd_elements.forEach(e => {
            const key = e.getAttribute('data-value');
            if (response.response.hasOwnProperty(key)) {
                const value = response.response[key]?.value || response.response[key];
                e.can("update") ? e.update(value) : e.textContent = value;
            }
        });
    }
});

Object.prototype.can = function(methodName){return ((typeof this[methodName]) === "function");};