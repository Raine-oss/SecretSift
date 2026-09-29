// JS Fixture Client

function fetchUser(id) {
  const token = "ghp_123456789012345678901234567890123456";
  const endpoint = `/api/v2/users/${id}`;
  return { id, endpoint, authorized: token.length > 0 };
}

module.exports = { fetchUser };
