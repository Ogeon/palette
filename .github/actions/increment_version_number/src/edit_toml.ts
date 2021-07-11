import * as toml from '@toml-tools/lexer';
import { IToken } from 'chevrotain';
import { promises as fs } from 'fs';
import * as tomlStringify from '@iarna/toml/stringify';

interface TokenPrimitive {
    index: number,
    value: boolean | number | string
}
type TokenValue = Map<string, TokenValue> | TokenPrimitive | TokenValue[];

interface FillMap {
    type: 'fill map',
    map: Map<string, TokenValue>
};
interface FillArray {
    type: 'fill array',
    array: TokenValue[]
};
interface AssignMapValue {
    type: 'assign map value',
    map: Map<string, TokenValue>,
    key: string
};
interface BeginAssignToMember {
    type: 'begin assign to member',
    isArray: boolean,
    map: Map<string, TokenValue>,
    key?: string
};
interface ExpectRSquare {
    type: 'expect RSquare'
}
type State = FillMap | FillArray | AssignMapValue | BeginAssignToMember | ExpectRSquare;

/**
 * A TOML file where primitive values (booleans, numbers, and strings) can be
 * replaced without affecting the overall formatting.
 */
export class TomlFile {
    /**
     * A tree representation of the file content, where values map to token
     * indices.
     */
    private valueMap: Map<string, TokenValue>;
    /**
     * The file in token form.
     */
    private tokens: IToken[];
    /**
     * The original file content.
     */
    private source: string;
    /**
     * A sparse array of substituted values. Each value is indexed by their
     * position in the token array.
     */
    private substitutions: (boolean | number | string)[] = [];

    private constructor(valueMap: Map<string, TokenValue>, tokens: IToken[], source: string) {
        this.valueMap = valueMap;
        this.tokens = tokens;
        this.source = source;
    }

    /**
     * Load a file as an editable TOML file.
     */
    static async load(file: string): Promise<TomlFile> {
        const source = (await fs.readFile(file)).toString('utf-8');
        const [valueMap, tokens] = parseSource(source);

        return new TomlFile(valueMap, tokens, source);
    }

    /**
     * Checks if there is a replaceable primitive value at the provided location
     * in the hierarchy.
     */
    hasPrimitive(path: (string | number)[]): boolean {
        let current: TokenValue = this.valueMap;

        for (const key of path) {
            if (current instanceof Map) {
                if (typeof key !== 'string') {
                    return false;
                }

                if (!current.has(key)) {
                    return false;
                }

                current = current.get(key)!;
            } else if (Array.isArray(current)) {
                if (typeof key !== 'number') {
                    return false;
                }

                if (current[key] === undefined) {
                    return false;
                }

                current = current[key];
            } else {
                return false;
            }
        }

        return !(current instanceof Map) && !Array.isArray(current);
    }

    /**
     * Get the current value at a location in the hierarchy, if it's primitive.
     * Throws an exception otherwise, or if it's not found.
     */
    getPrimitive(path: (string | number)[]): boolean | string | number {
        let current: TokenValue = this.valueMap;
        let traversed = '';

        for (const key of path) {
            if (current instanceof Map) {
                if (typeof key !== 'string') {
                    throw new Error(`expected a string index into ${traversed} but found ${key}`);
                }

                if (!current.has(key)) {
                    throw new Error(`${key} does not exist in ${traversed}`);
                }

                current = current.get(key)!;
                traversed = traversed.length ? `${traversed}.${key}` : key;
            } else if (Array.isArray(current)) {
                if (typeof key !== 'number') {
                    throw new Error(`expected a number index into ${traversed} but found ${key}`);
                }

                if (current[key] === undefined) {
                    throw new Error(`[${key}] does not exist in ${traversed}`);
                }

                current = current[key];
                traversed = `${traversed}[${key}]`;
            } else {
                throw new Error(`${traversed} is not an object or array`);
            }
        }

        if (!(current instanceof Map) && !Array.isArray(current)) {
            return current.value;
        } else {
            throw new Error(`expected ${traversed} to be a primitive value`)
        }
    }

    /**
     * Replace a value at a location in the hierarchy, if it's primitive. Throws
     * an exception otherwise, or if it's not found.
     */
    setPrimitive(path: (string | number)[], newValue: boolean | string | number) {
        let current: TokenValue = this.valueMap;
        let traversed = '';

        for (const key of path) {
            if (current instanceof Map) {
                if (typeof key !== 'string') {
                    throw new Error(`expected a string index into ${traversed} but found ${key}`);
                }

                if (!current.has(key)) {
                    throw new Error(`${key} does not exist in ${traversed}`);
                }

                current = current.get(key)!;
                traversed = traversed.length ? `${traversed}.${key}` : key;
            } else if (Array.isArray(current)) {
                if (typeof key !== 'number') {
                    throw new Error(`expected a number index into ${traversed} but found ${key}`);
                }

                if (current[key] === undefined) {
                    throw new Error(`[${key}] does not exist in ${traversed}`);
                }

                current = current[key];
                traversed = `${traversed}[${key}]`;
            } else {
                throw new Error(`${traversed} is not an object or array`);
            }
        }

        if (!(current instanceof Map) && !Array.isArray(current)) {
            current.value = newValue;
            this.substitutions[current.index] = newValue;
        } else {
            throw new Error(`expected ${traversed} to be a primitive value`)
        }
    }

    /**
     * Render the altered file content as a text string, that can be written to
     * a file.
     */
    render(): string {
        let result = '';
        let begin = 0;
        let end = 0;

        for (const [index, token] of this.tokens.entries()) {
            if (this.substitutions[index] != undefined) {
                result += this.source.substring(begin, token.startOffset);
                result += tomlStringify.value(this.substitutions[index])
                begin = token.endOffset! + 1;
                end = begin;
            } else {
                end = token.endOffset! + 1;
            }
        }

        if (begin != end) {
            result += this.source.substring(begin, end);
        }

        return result;
    }
}

/**
 * Parses the file content into a token array and a hierarchy map. It doesn't do
 * any detailed validation and mostly assumes the file valid.
 */
function parseSource(source: string): [Map<string, TokenValue>, IToken[]] {
    const result = toml.tokenize(source);
    const valueMap = new Map();
    let stateStack: State[] = [{ type: 'fill map', map: valueMap }];
    for (const [index, token] of result.tokens.entries()) {
        if (!token.tokenType) {
            throw Error(`missing token type for ${token.image}`);
        }

        const currentState = stateStack[stateStack.length - 1];

        switch (token.tokenType.name) {
            case 'UnquotedKey':
                switch (currentState.type) {
                    case 'fill map':
                        stateStack.push({
                            type: 'assign map value',
                            map: currentState.map,
                            key: token.image
                        });
                        break;
                    case 'begin assign to member':
                        if (currentState.key) {
                            if (currentState.map.has(currentState.key)) {
                                const child = currentState.map.get(currentState.key);
                                if (child instanceof Map) {
                                    currentState.map = child;
                                } else {
                                    throw Error(`expected ${currentState.key} to be an object`);
                                }
                            } else {
                                const newMap = new Map();
                                currentState.map.set(currentState.key, newMap);
                                currentState.map = newMap;
                            }
                        }

                        currentState.key = token.image;
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'True':
                switch (currentState.type) {
                    case 'assign map value':
                        currentState.map.set(currentState.key, { index, value: true });
                        stateStack.pop();
                        break;
                    case 'fill array':
                        currentState.array.push({ index, value: true });
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'False':
                switch (currentState.type) {
                    case 'assign map value':
                        currentState.map.set(currentState.key, { index, value: false });
                        stateStack.pop();
                        break;
                    case 'fill array':
                        currentState.array.push({ index, value: false });
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'BasicString': {
                const stringValue = token.image.substring(1, token.image.length - 1);
                switch (currentState.type) {
                    case 'assign map value':
                        currentState.map.set(currentState.key, { index, value: stringValue });
                        stateStack.pop();
                        break;
                    case 'fill array':
                        currentState.array.push({ index, value: stringValue });
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            }
            case 'LSquare':
                switch (currentState.type) {
                    case 'fill map':
                        stateStack = [
                            { type: 'fill map', map: valueMap },
                            { type: 'begin assign to member', isArray: false, map: valueMap }
                        ];
                        break;
                    case 'begin assign to member':
                        currentState.isArray = true;
                        break;
                    case 'assign map value': {
                        const newArray: TokenValue[] = [];
                        currentState.map.set(currentState.key, newArray);
                        stateStack.pop();
                        stateStack.push({ type: 'fill array', array: newArray });
                        break;
                    }
                    case 'fill array': {
                        const newArray: TokenValue[] = [];
                        currentState.array.push(newArray);
                        stateStack.pop();
                        stateStack.push({ type: 'fill array', array: newArray });
                        break;
                    }
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'RSquare':
                switch (currentState.type) {
                    case 'begin assign to member':
                        const newMap = new Map();

                        if (!currentState.key) {
                            throw Error('expected a key when adding a member');
                        }

                        if (currentState.isArray) {
                            if (!currentState.map.has(currentState.key)) {
                                currentState.map.set(currentState.key, []);
                            }

                            const memberArray = currentState.map.get(currentState.key);
                            if (!Array.isArray(memberArray)) {
                                throw Error(`expected ${currentState.key} to be an array`);
                            }
                            memberArray.push(newMap);
                        } else {
                            currentState.map.set(currentState.key, newMap);
                        }

                        stateStack.pop();

                        stateStack.push({ type: 'fill map', map: newMap });

                        if (currentState.isArray) {
                            stateStack.push({ type: 'expect RSquare' });
                        }
                        break;
                    case 'fill array':
                    case 'expect RSquare':
                        stateStack.pop();
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'LCurly':
                switch (currentState.type) {
                    case 'assign map value': {
                        const newMap = new Map();
                        currentState.map.set(currentState.key, newMap);
                        stateStack.pop();
                        stateStack.push({ type: 'fill map', map: newMap });
                        break;
                    }
                    case 'fill array': {
                        const newMap = new Map();
                        currentState.array.push(newMap);
                        stateStack.push({ type: 'fill map', map: newMap });
                        break;
                    }
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'RCurly':
                switch (currentState.type) {
                    case 'fill map':
                        stateStack.pop();
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'Dot':
                switch (currentState.type) {
                    case 'begin assign to member':
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'Comma':
                switch (currentState.type) {
                    case 'fill array':
                    case 'fill map':
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'KeyValSep':
                switch (currentState.type) {
                    case 'assign map value':
                        break;
                    default:
                        unexpectedTokenError(token.tokenType.name, currentState.type, token);
                        break;
                }
                break;
            case 'Newline':
            case 'Comment':
                break;
            default:
                throw Error(`unexpected token type ${token.tokenType.name}`);
        }
    }

    return [valueMap, result.tokens];
}

/**
 * Throw a formatted parse error.
 */
function unexpectedTokenError(tokenType: string, stateType: string, token: IToken) {
    throw Error(`unexpected ${tokenType} during ${stateType}: ${token.image}`);
}
