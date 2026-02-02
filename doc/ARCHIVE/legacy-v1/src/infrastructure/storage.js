/**
 * Versioned localStorage wrapper.
 *
 * Every persistent store in Tech Pulse is created through createStore().
 * This module owns the storage abstraction and migration logic.
 * It must NOT know about any specific data schema.
 *
 * This module is FROZEN: replacing localStorage with IndexedDB
 * should require changes only here.
 */

const STORAGE_PREFIX = 'techpulse_';

/**
 * Creates a versioned persistent store backed by localStorage.
 *
 * @param {string} key             - Storage key (without prefix)
 * @param {Object} options
 * @param {number} options.version - Current schema version (positive integer)
 * @param {Object} [options.migrations] - Map of version → migration function.
 *   Each migration receives data at version N-1 and must return data at version N.
 *   Migrations are pure functions: no I/O, no side effects.
 *   Migrations are never deleted — the full chain must be preserved.
 * @param {Function} options.defaultValue - Factory returning the default empty state
 * @returns {Readonly<{ read(): any, write(data: any): void, clear(): void }>}
 */
export function createStore(key, { version, migrations = {}, defaultValue }) {
  if (typeof key !== 'string' || key.length === 0) {
    throw new Error('createStore: key must be a non-empty string');
  }
  if (typeof version !== 'number' || version < 1 || !Number.isInteger(version)) {
    throw new Error('createStore: version must be a positive integer');
  }
  if (typeof defaultValue !== 'function') {
    throw new Error('createStore: defaultValue must be a factory function');
  }

  const fullKey = STORAGE_PREFIX + key;

  function read() {
    const raw = localStorage.getItem(fullKey);
    if (raw === null) return defaultValue();

    let data;
    try {
      data = JSON.parse(raw);
    } catch {
      console.warn(`[storage] Corrupted data for "${key}", resetting to default.`);
      return defaultValue();
    }

    return applyMigrations(data, version, migrations, key);
  }

  function write(data) {
    const versioned = { ...data, _version: version };
    localStorage.setItem(fullKey, JSON.stringify(versioned));
  }

  function clear() {
    localStorage.removeItem(fullKey);
  }

  return Object.freeze({ read, write, clear });
}

/**
 * Apply migration chain from data's current version to target version.
 * @private
 */
function applyMigrations(data, targetVersion, migrations, key) {
  let current = data._version || 0;

  while (current < targetVersion) {
    current++;
    const migrator = migrations[current];
    if (migrator) {
      try {
        data = migrator(data);
      } catch (err) {
        console.warn(`[storage] Migration of "${key}" to v${current} failed:`, err);
        // Stop migrating rather than corrupt data further.
        // Return data at whatever version we reached.
        data._version = current - 1;
        return data;
      }
    }
    data._version = current;
  }

  return data;
}
