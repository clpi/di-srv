 -- TABLE Users; (id ...)
 -- TABLE Records; (id ...)
 -- TABLE Items; (id ...)
 -- TABLE Fields; (id ...)
 -- TABLE EntryTypes; (id ..)
 -- TABLE RecordItemLinks; (id, rid, iid)
 -- TABLE ItemFieldLinks; (id, iid, fid)
 -- TABLE FieldEntryLinks; id, fid, eid)
 -- TABLE UserGroupLinks; (id, uid, gid)
 -- TABLE UserRecordLinks; (id, uid, rid)

-- Get all records associated with a single user (with id)
SELECT * FROM Records r
    JOIN UserRecordLinks url ON (r.id = url.rid)
    JOIN Users u ON (u.id = url.uid)
    WHERE u.id = ? --AND r.uid != u.id (if only want recs shared w/ user)

-- GET ALL users associated with a single record (with id)
SELECT * FROM Users u
    JOIN UserRecordLinks url ON (u.id = url.uid)
    JOIN Records r ON (r.id = url.rid)
    WHERE r.id = ?

-- GET ALL users associated with any record that is associated with an item
SELECT * FROM Users u
    JOIN UserRecordLinks url ON (u.id = url.uid)
    JOIN Records r ON (r.id = url.rid)
    WHERE r.id IN 
    (SELECT id FROM Records r1
        JOIN RecordItemLinks ril ON (r1.id = ril.rid)
        JOIN Items i on (i.id = ril.iid)
        WHERE i.id = ?)

-- GET ALL fields associated with a certain item
SELECT * FROM Fields f
    JOIN ItemFieldLinks ifl ON (f.id = ifl.fid)
    JOIN Items i ON (i.id = ifl.iid)
    WHERE i.id = ?

-- GET ALL Users who are associated with records who have an item
SELECT * FROM Users u
    JOIN UserRecordLinks url ON (u.id = url.uid)
    JOIN Records r ON (r.id = url.rid)
    WHERE r.id IN 
    (SELECT id FROM Records r1
        JOIN RecordItemLinks ril ON (r1.id = ril.rid)
        JOIN Items i on (i.id = ril.iid)
        WHERE i.id = ?)

-- GET all items who share a specific field
SELECT * FROM Items i
    JOIN ItemFieldLinks ifl ON (i.id = ifl.iid)
    JOIN Fields f ON (f.id = ifl.fid)
    WHERE f.id = ? --AND r.uid != u.id (if only want recs shared w/ user)

-- GET ALL records who share a specific item
SELECT * FROM Records r
    JOIN RecordItemLink ril ON (r.id = ril.rid)
    JOIN Items i ON (i.id = ril.iid)
    WHERE i.id = ? --AND r.uid != u.id (if only want recs shared w/ user)

-- GET ALL Users associated with a specific group
SELECT * FROM Users u
    JOIN UserGroupLinks ugl ON (u.id = ugl.uid)
    JOIN Groups g ON (g.id = ugl.gid)
    WHERE g.id = ? --AND r.uid != u.id (if only want recs shared w/ user)

-- GET ALL Entries by a user which have filled out a specific field
-- NOTE Check this one
SELECT * From EntryEntries ee
    JOIN EntryTypes et ON (ee.etid = et.id)
    JOIN FieldEntryLinks ON (et.id = fel.etid)
    JOIN Fields f ON (f.id = fel.fid)
    WHERE f.id = ?

-- GET ALL Entries by a user which have filled out *any* fields associatd with an item

-- GET ALL Entries by a user which have filled out ALL fields associated with an item

-- 
