from pathlib import Path


def replace_once(path: str, old: str, new: str, label: str) -> None:
    p = Path(path)
    s = p.read_text()
    if new in s and old not in s:
        return
    count = s.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    p.write_text(s.replace(old, new, 1))


replace_once(
    "infrastructure/tari_script/src/op_codes.rs",
    "        .chunks_exact(PUBLIC_KEY_LENGTH)\n        .take(num)\n        .map(CompressedKey::from_canonical_bytes)",
    "        .as_chunks::<PUBLIC_KEY_LENGTH>()\n        .0\n        .iter()\n        .take(num)\n        .map(|bytes| CompressedKey::from_canonical_bytes(&bytes[..]))",
    "op_codes",
)

replace_once(
    "base_layer/wallet/src/transaction_service/storage/sqlite_db.rs",
    "        .chunks_exact(32)\n        .filter_map(|chunk| FixedHash::try_from(chunk).ok())",
    "        .as_chunks::<32>()\n        .0\n        .iter()\n        .filter_map(|chunk| FixedHash::try_from(&chunk[..]).ok())",
    "sqlite_db",
)

p = Path("my_escrow_contract/src/lib.rs")
s = p.read_text()
if not ("Copyright" in s and "The Tari Project" in s):
    header = """// Copyright 2026. The Tari Project
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

"""
    p.write_text(header + s)
