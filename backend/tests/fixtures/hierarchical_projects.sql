-- Hierarchical Project Structure Fixture

-- Root Project (Container)
INSERT INTO projects (name, full_path, parent_id, created_at, updated_at, is_leaf)
VALUES ('Vehicles', '/projects/Vehicles', NULL, 1000000000, 1000000000, 0);

-- Sub-Project (Container)
INSERT INTO projects (name, full_path, parent_id, created_at, updated_at, is_leaf)
VALUES ('Cars', '/projects/Vehicles/Cars', (SELECT id FROM projects WHERE name = 'Vehicles'), 1000000000, 1000000000, 0);

-- Leaf Project (Content)
INSERT INTO projects (name, full_path, parent_id, created_at, updated_at, is_leaf)
VALUES ('Sports Car', '/projects/Vehicles/Cars/Sports Car', (SELECT id FROM projects WHERE name = 'Cars'), 1000000000, 1000000000, 1);

-- Another Leaf Project
INSERT INTO projects (name, full_path, parent_id, created_at, updated_at, is_leaf)
VALUES ('Truck', '/projects/Vehicles/Truck', (SELECT id FROM projects WHERE name = 'Vehicles'), 1000000000, 1000000000, 1);

-- Add some files to Leaf Projects
INSERT INTO stl_files (project_id, filename, file_path, file_size, created_at, updated_at)
VALUES (
    (SELECT id FROM projects WHERE name = 'Sports Car'),
    'body.stl',
    '/projects/Vehicles/Cars/Sports Car/body.stl',
    1024,
    1000000000,
    1000000000
);

INSERT INTO stl_files (project_id, filename, file_path, file_size, created_at, updated_at)
VALUES (
    (SELECT id FROM projects WHERE name = 'Truck'),
    'cab.stl',
    '/projects/Vehicles/Truck/cab.stl',
    2048,
    1000000000,
    1000000000
);
