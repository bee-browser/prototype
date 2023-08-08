// deno-fmt-ignore-file
// deno-lint-ignore-file
// This code was bundled using `deno bundle` and it's not recommended to edit it manually

'use strict';
function h(tagName, attrs = [], ...children) {
    const elem = document.createElement(tagName);
    for (const attr of Object.entries(attrs)){
        elem.setAttribute(attr[0], attr[1]);
    }
    for (const child of children){
        elem.appendChild(child);
    }
    return elem;
}
function t(text) {
    return document.createTextNode(text);
}
class EventEmitter {
    events = new Map();
    maxListeners;
    #defaultMaxListeners = 10;
    get defaultMaxListeners() {
        return this.#defaultMaxListeners;
    }
    set defaultMaxListeners(n) {
        if (Number.isInteger(n) || n < 0) {
            const error = new RangeError('The value of "defaultMaxListeners" is out of range. It must be a non-negative integer. Received ' + n + '.');
            throw error;
        }
        this.#defaultMaxListeners = n;
    }
    addListener(eventName, listener) {
        return this.on(eventName, listener);
    }
    emit(eventName, ...args) {
        const listeners = this.events.get(eventName);
        if (listeners === undefined) {
            if (eventName === 'error') {
                const error = args[0];
                if (error instanceof Error) throw error;
                throw new Error('Unhandled error.');
            }
            return false;
        }
        const copyListeners = [
            ...listeners
        ];
        for (const listener of copyListeners){
            listener.apply(this, args);
        }
        return true;
    }
    setMaxListeners(n) {
        if (!Number.isInteger(n) || n < 0) {
            throw new RangeError('The value of "n" is out of range. It must be a non-negative integer. Received ' + n + '.');
        }
        this.maxListeners = n;
        return this;
    }
    getMaxListeners() {
        if (this.maxListeners === undefined) {
            return this.defaultMaxListeners;
        }
        return this.maxListeners;
    }
    listenerCount(eventName) {
        const events = this.events.get(eventName);
        return events === undefined ? 0 : events.length;
    }
    eventNames() {
        return Reflect.ownKeys(this.events);
    }
    listeners(eventName) {
        const listeners = this.events.get(eventName);
        return listeners === undefined ? [] : listeners;
    }
    off(eventName, listener) {
        return this.removeListener(eventName, listener);
    }
    on(eventName, listener, prepend) {
        if (this.events.has(eventName) === false) {
            this.events.set(eventName, []);
        }
        const events = this.events.get(eventName);
        if (prepend) {
            events.unshift(listener);
        } else {
            events.push(listener);
        }
        if (eventName !== "newListener" && this.events.has("newListener")) {
            this.emit('newListener', eventName, listener);
        }
        const maxListener = this.getMaxListeners();
        const eventLength = events.length;
        if (maxListener > 0 && eventLength > maxListener && !events.warned) {
            events.warned = true;
            const warning = new Error(`Possible EventEmitter memory leak detected.
         ${this.listenerCount(eventName)} ${eventName.toString()} listeners.
         Use emitter.setMaxListeners() to increase limit`);
            warning.name = "MaxListenersExceededWarning";
            console.warn(warning);
        }
        return this;
    }
    removeAllListeners(eventName) {
        const events = this.events;
        if (!events.has('removeListener')) {
            if (arguments.length === 0) {
                this.events = new Map();
            } else if (events.has(eventName)) {
                events.delete(eventName);
            }
            return this;
        }
        if (arguments.length === 0) {
            for (const key of events.keys()){
                if (key === 'removeListener') continue;
                this.removeAllListeners(key);
            }
            this.removeAllListeners('removeListener');
            this.events = new Map();
            return this;
        }
        const listeners = events.get(eventName);
        if (listeners !== undefined) {
            listeners.map((listener)=>{
                this.removeListener(eventName, listener);
            });
        }
        return this;
    }
    removeListener(eventName, listener) {
        const events = this.events;
        if (events.size === 0) return this;
        const list = events.get(eventName);
        if (list === undefined) return this;
        const index = list.findIndex((item)=>item === listener || item.listener === listener);
        if (index === -1) return this;
        list.splice(index, 1);
        if (list.length === 0) this.events.delete(eventName);
        if (events.has('removeListener')) {
            this.emit('removeListener', eventName, listener);
        }
        return this;
    }
    once(eventName, listener) {
        this.on(eventName, this.onceWrap(eventName, listener));
        return this;
    }
    onceWrap(eventName, listener) {
        const wrapper = function(...args) {
            this.context.removeListener(this.eventName, this.wrapedListener);
            this.listener.apply(this.context, args);
        };
        const wrapperContext = {
            eventName: eventName,
            listener: listener,
            wrapedListener: wrapper,
            context: this
        };
        const wrapped = wrapper.bind(wrapperContext);
        wrapperContext.wrapedListener = wrapped;
        wrapped.listener = listener;
        return wrapped;
    }
    prependListener(eventName, listener) {
        return this.on(eventName, listener, true);
    }
    prependOnceListener(eventName, listener) {
        this.prependListener(eventName, this.onceWrap(eventName, listener));
        return this;
    }
    rawListeners(eventName) {
        const events = this.events;
        if (events === undefined) return [];
        const listeners = events.get(eventName);
        if (listeners === undefined) return [];
        return [
            ...listeners
        ];
    }
}
'use strict';
class Widget extends EventEmitter {
    constructor(){
        super();
        this.elem_ = null;
    }
    render() {
        throw new Error('must be override');
    }
    clear() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.innerHTML = '';
    }
    show() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.remove('hide');
    }
    hide() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.add('hide');
    }
    select() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.add('selected');
    }
    deselect() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.remove('selected');
    }
    scrollIntoView() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.scrollIntoViewIfNeeded(true);
    }
    hasRendered() {
        return this.elem_ !== null;
    }
}
class ParserView extends Widget {
    constructor(){
        super();
        this.tokenView_ = new TokenView();
        this.stackView_ = new StackView();
    }
    render() {
        this.elem_ = h('div', {
            id: 'parser-view'
        }, this.tokenView_.render(), this.stackView_.render());
        return this.elem_;
    }
    feed(data) {
        switch(data.opcode){
            case 'token':
                this.tokenView_.setToken({
                    kind: data['token.kind'],
                    lexeme: data['token.lexeme']
                });
                break;
            case 'push':
                this.stackView_.pushState(data.state);
                break;
            case 'pop':
                this.stackView_.popStates(data.num_states);
                break;
            case 'accept':
                break;
            case 'shift':
                break;
            case 'reduce':
                break;
        }
    }
}
class TokenView extends Widget {
    constructor(){
        super();
        this.token_ = null;
    }
    render() {
        this.elem_ = h('div', {
            id: 'token'
        }, h('div', {
            id: 'token-kind'
        }), h('div', {
            id: 'token-lexeme'
        }));
        return this.elem_;
    }
    setToken(token) {
        this.token_ = token;
        document.getElementById('token-kind').innerHTML = '';
        document.getElementById('token-lexeme').innerHTML = '';
        if (this.token_) {
            document.getElementById('token-kind').appendChild(t(this.token_.kind));
            document.getElementById('token-lexeme').appendChild(t(this.token_.lexeme));
        }
    }
}
class StackView extends Widget {
    constructor(){
        super();
        this.views_ = [];
    }
    render() {
        this.elem_ = h('div', {
            id: 'parser-stack'
        });
        for (const view of this.views_){
            this.elem_.appendChild(view.render());
        }
        return this.elem_;
    }
    pushState(state) {
        const items = state.split(', ');
        const view = new StateView(items);
        this.elem_.appendChild(view.render());
        this.views_.push(view);
    }
    popStates(n) {
        while(n > 0){
            const view = this.views_.pop();
            this.elem_.removeChild(view.elem_);
            n--;
        }
    }
}
class StateView extends Widget {
    constructor(items){
        super();
        this.items_ = items;
    }
    render() {
        this.elem_ = h('div', {
            class: 'parser-state'
        });
        for (const item of this.items_){
            this.elem_.appendChild(h('div', {
                class: 'parser-state-item'
            }, t(item)));
        }
        return this.elem_;
    }
}
class LexerView extends Widget {
    constructor(){
        super();
        this.cursorPos_ = 0;
        this.cursorTokenEnd_ = 0;
    }
    render() {
        this.elem_ = h('div', {
            id: 'lexer-view'
        }, h('div', {
            id: 'lexer-cursor'
        }, t('0, 0')), h('div', {
            id: 'lexer-state'
        }), h('div', {
            id: 'lexical-goal'
        }), h('div', {
            id: 'candidate-token'
        }));
        return this.elem_;
    }
    feed(data) {
        switch(data.opcode){
            case 'set_goal':
                this.setGoal_(data.goal);
                break;
            case 'state':
                this.setState_(data.state);
                break;
            case 'char':
                break;
            case 'unicode-set':
                break;
            case 'candidate':
                this.setToken_({
                    kind: data['token.kind'],
                    lexeme: data['token.lexeme']
                });
                break;
            case 'consume':
                this.cursorTokenEnd_ = data['cursor.token_end'];
                this.updateCursor_();
                break;
            case 'lookahead':
                break;
            case 'advance':
                this.cursorPos_ = data['cursor.pos'];
                this.updateCursor_();
                break;
        }
    }
    updateCursor_() {
        document.getElementById('lexer-cursor').innerHTML = `${this.cursorPos_}, ${this.cursorTokenEnd_}`;
    }
    setState_(state) {
        if (state === 'State(0)') {
            this.setToken_(null);
        }
        document.getElementById('lexer-state').innerHTML = '';
        document.getElementById('lexer-state').appendChild(t(state));
    }
    setGoal_(goal) {
        document.getElementById('lexical-goal').innerHTML = '';
        document.getElementById('lexical-goal').appendChild(t(goal));
    }
    setToken_(token) {
        document.getElementById('candidate-token').innerHTML = '';
        if (token) {
            document.getElementById('candidate-token').appendChild(t(token.lexeme));
        }
    }
}
class MainView extends Widget {
    constructor(){
        super();
        this.pc_ = 0;
        this.logs_ = [];
        this.parserView_ = new ParserView();
        this.lexerView_ = new LexerView();
        this.on('log', this.handleLog_.bind(this));
    }
    render() {
        this.elem_ = h('div', {
            id: 'main-view'
        }, this.parserView_.render(), this.lexerView_.render());
        return this.elem_;
    }
    start() {
        const es = new EventSource('/logs');
        es.addEventListener('spawned', (event)=>{
            console.debug('spawned');
        });
        es.addEventListener('log', (event)=>{
            const log = JSON.parse(event.data);
            this.emit('log', log);
        });
        es.addEventListener('terminated', (event)=>{
            console.debug('terminated');
            event.target.close();
        });
        es.addEventListener('error', (event)=>{
            console.error('error');
            event.target.close();
        });
    }
    dispatch_() {
        const log = this.logs_[this.pc_];
        if (log === undefined) {
            return;
        }
        this.pc_++;
        switch(log.type){
            case 'parser':
                this.parserView_.feed(log.data);
                break;
            case 'lexer':
                this.lexerView_.feed(log.data);
                break;
        }
    }
    handleLog_(log) {
        if (log.target.startsWith('bee_jsparser::parser')) {
            switch(log.level){
                case 'TRACE':
                    this.logs_.push({
                        type: 'parser',
                        level: 'trace',
                        data: log.fields
                    });
                    break;
            }
        }
        if (log.target.startsWith('bee_jsparser::lexer')) {
            switch(log.level){
                case 'TRACE':
                    this.logs_.push({
                        type: 'lexer',
                        level: 'trace',
                        data: log.fields
                    });
                    break;
            }
        }
        this.dispatch_();
    }
}
'use strict';
const widget = new MainView;
document.body.appendChild(widget.render());
widget.start();
