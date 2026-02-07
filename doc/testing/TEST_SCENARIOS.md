# Test Scenarios for Auto-Refresh Feature

This document outlines test scenarios to verify that the file system auto-refresh feature works correctly.

---

## Scenario 1: Auto-Refresh on Picker Activation

**Goal:** Verify that pressing `@` always shows the latest files/folders.

### Steps:

1. **Start the app:**
   ```bash
   cargo run
   ```

2. **In another terminal, create a new file:**
   ```bash
   touch test-file-new.txt
   ```

3. **Back in the app, press `@`:**
   - The filesystem is automatically rescanned
   - You should see `test-file-new.txt` in the file list

4. **Expected Result:**
   ✅ New file appears immediately in the picker

### Verification:
- Type `@` → `f` → `test-file`
- The newly created `test-file-new.txt` should be visible and selectable

---

## Scenario 2: Manual Refresh with F5

**Goal:** Verify that F5 refreshes the file list while picker is open.

### Steps:

1. **Start the app and open picker:**
   - Type `@`
   - Press `f` (select file mode)
   - Note the current file count (e.g., `[1/10]`)

2. **Keep picker open, switch to another terminal:**
   ```bash
   mkdir new-test-folder
   touch new-test-folder/inside.txt
   ```

3. **Back in the app (with picker still open):**
   - Press `F5`
   - The file count should update (e.g., `[1/12]`)

4. **Search for the new folder:**
   - Press `ESC` to close picker
   - Type `@` → `d` (folder mode)
   - Type `new-test`
   - The new folder should appear

### Expected Result:
✅ F5 updates the file list without closing the picker
✅ New files/folders are immediately visible after F5

---

## Scenario 3: Create Files Between Picker Sessions

**Goal:** Verify that each `@` press gives fresh data.

### Steps:

1. **First picker session:**
   - Type `@` → `f`
   - Note files starting with "test" (e.g., 2 files)
   - Press `ESC` to close

2. **Create new files:**
   ```bash
   touch test-alpha.txt
   touch test-beta.txt
   touch test-gamma.txt
   ```

3. **Second picker session:**
   - Type `@` → `f` → `test`
   - Should now see 5 files starting with "test"

### Expected Result:
✅ Second picker session shows all 5 test files
✅ No need to restart the app

---

## Scenario 4: Delete Files and Verify Removal

**Goal:** Verify that deleted files disappear from the picker.

### Steps:

1. **Create a temporary file:**
   ```bash
   touch temp-delete-me.txt
   ```

2. **Verify it exists in picker:**
   - Type `@` → `f` → `delete`
   - Should see `temp-delete-me.txt`
   - Press `ESC`

3. **Delete the file:**
   ```bash
   rm temp-delete-me.txt
   ```

4. **Check picker again:**
   - Type `@` → `f` → `delete`
   - File should NOT appear

### Expected Result:
✅ Deleted file is no longer in the picker
✅ Auto-refresh detects file removal

---

## Scenario 5: Hidden Files Appear Immediately

**Goal:** Verify that hidden files (starting with `.`) are indexed and refreshed.

### Steps:

1. **Create a hidden file:**
   ```bash
   touch .my-hidden-config
   ```

2. **Open picker:**
   - Type `@` → `f` → `.my-hidden`
   - Hidden file should appear: `◆ .my-hidden-config`

3. **Create another hidden file while picker is open:**
   ```bash
   touch .another-hidden
   ```

4. **In the app, press F5:**
   - Both hidden files should now be visible

### Expected Result:
✅ Hidden files are indexed and searchable
✅ F5 picks up newly created hidden files

---

## Scenario 6: Nested Folder Creation

**Goal:** Verify that nested folders are indexed on refresh.

### Steps:

1. **Create nested structure:**
   ```bash
   mkdir -p level1/level2/level3
   touch level1/level2/level3/deep-file.txt
   ```

2. **Open picker:**
   - Type `@` → `d` → `level`
   - Should see `level1`, `level2`, `level3` folders

3. **Try to find the deep file:**
   - Type `@` → `f` → `deep`
   - Should see `deep-file.txt`

### Expected Result:
✅ Nested folders up to 10 levels are indexed
✅ Files in nested folders are searchable

---

## Scenario 7: Large Directory Performance

**Goal:** Verify that auto-refresh is fast even with many files.

### Steps:

1. **Create many files:**
   ```bash
   for i in {1..100}; do touch "perf-test-$i.txt"; done
   ```

2. **Open picker multiple times:**
   - Type `@` → `f` → `perf`
   - Note the speed
   - Press `ESC`
   - Repeat 5 times

### Expected Result:
✅ Picker opens quickly (< 100ms for typical projects)
✅ No noticeable lag on subsequent `@` presses
✅ All 100 files are indexed and searchable

### Cleanup:
```bash
rm perf-test-*.txt
```

---

## Scenario 8: Target Directory Access (Previously Hidden)

**Goal:** Verify that `target/` folder is now accessible.

### Steps:

1. **Build the project (creates target/):**
   ```bash
   cargo build
   ```

2. **Search for target folder:**
   - Type `@` → `d` → `target`
   - Should see `▸ target` in results

3. **Search for files in target:**
   - Type `@` → `f` → `debug`
   - Should see files from `target/debug/`

### Expected Result:
✅ `target/` folder is visible and selectable
✅ Files inside `target/` are indexed
✅ No filtering of build artifacts

---

## Scenario 9: Refresh During Active Search

**Goal:** Verify F5 works while typing a search query.

### Steps:

1. **Open picker and start searching:**
   - Type `@` → `f` → `test`
   - See current results (e.g., 3 matches)

2. **Without closing picker, create new file in another terminal:**
   ```bash
   touch test-during-search.txt
   ```

3. **Press F5 in the app:**
   - Results should update to 4 matches
   - New file should appear in the filtered list

### Expected Result:
✅ F5 refreshes without losing your search query
✅ Filtered results update immediately
✅ Selection stays on a valid item

---

## Scenario 10: Empty Directory Handling

**Goal:** Verify empty directories are handled correctly.

### Steps:

1. **Create empty directories:**
   ```bash
   mkdir empty-folder-1
   mkdir empty-folder-2
   mkdir empty-folder-3
   ```

2. **Search for them:**
   - Type `@` → `d` → `empty`
   - All 3 empty folders should appear

3. **Delete one:**
   ```bash
   rmdir empty-folder-2
   ```

4. **Refresh and search again:**
   - Type `@` → `d` → `empty`
   - Should only see 2 folders now

### Expected Result:
✅ Empty folders are indexed
✅ Deleted empty folders are removed on refresh

### Cleanup:
```bash
rmdir empty-folder-1 empty-folder-3
```

---

## Performance Benchmarks

### Expected Performance:

| Files/Folders | Scan Time | Refresh Time |
|---------------|-----------|--------------|
| < 100         | < 10ms    | < 10ms       |
| 100-1000      | < 50ms    | < 50ms       |
| 1000-5000     | < 200ms   | < 200ms      |
| 5000+         | < 500ms   | < 500ms      |

*Note: Times are approximate and depend on system/disk speed*

---

## Edge Cases to Test

### Case 1: Symbolic Links
```bash
ln -s /some/other/path symlink-test
```
- Verify symlinks appear in picker
- They should be treated as their target type (file/folder)

### Case 2: Permission Issues
```bash
mkdir no-access
chmod 000 no-access
```
- Picker should skip folders it can't read
- No crashes or errors

### Case 3: Very Long Filenames
```bash
touch "this-is-a-very-long-filename-that-goes-on-and-on-and-might-cause-display-issues.txt"
```
- Long names should be handled gracefully
- UI should not break

### Case 4: Special Characters
```bash
touch "file with spaces.txt"
touch "file-with-dashes.txt"
touch "file_with_underscores.txt"
```
- All special characters should work in search
- File paths should be inserted correctly

---

## Continuous Testing Workflow

### Quick Test Loop:

1. Start app: `cargo run`
2. In another terminal: `touch quick-test-$(date +%s).txt`
3. In app: `@` → `f` → `quick-test`
4. Verify new file appears
5. Repeat to test consistency

### Automation Ideas:

Create a test script:
```bash
#!/bin/bash
# create-test-files.sh

for i in {1..10}; do
    sleep 1
    touch "auto-test-$i-$(date +%s).txt"
    echo "Created file $i"
done
```

Run this while using the app to test continuous updates.

---

## Known Limitations

1. **Max depth: 10 levels** - Files deeper than 10 levels are not indexed
2. **No real-time watching** - Changes only detected on `@` press or `F5`
3. **Large directories** - Scanning 10,000+ files may take a few hundred milliseconds

---

## Summary

✅ **Auto-refresh on `@`** - Always shows latest state
✅ **Manual F5 refresh** - Update while picker is open
✅ **No exclusions** - All files/folders visible (including `target`, `.git`, etc.)
✅ **Fast performance** - Typical projects scan in < 50ms
✅ **Robust** - Handles edge cases gracefully

The auto-refresh feature ensures users always work with up-to-date file listings without needing to restart the application.