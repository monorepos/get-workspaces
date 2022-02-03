const { getWorkspaces } = require('./lib/index.node');

/**
 * @typedef {Object} Workspace
 * @property {string} dir
 * @property {Record<string, any>} packageJson
 */

/**
 * Gets workspaces from the cwd
 * @param {string} cwd 
 * @returns {Workspace[]}
 */
module.exports.getWorkspaces = (cwd) => {
  const workspaces = getWorkspaces(cwd || process.cwd());
  const workspacesJSON = JSON.parse(workspaces);

  return workspacesJSON;
};

