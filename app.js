import {factorial} from './factorial.rs';

function update() {
  window.output.value = factorial(window.input.value);
}

window.input.addEventListener('change', update);
update();
