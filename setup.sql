-- Devices
CREATE TABLE IF NOT EXISTS device (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);  

-- Button
CREATE TABLE IF NOT EXISTS button (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  pulses TEXT NOT NULL,
  device INTEGER NOT NULL,
  
  FOREIGN KEY (device)
    REFERENCES device (id)
);  

-- Sequence
CREATE TABLE IF NOT EXISTS sequence (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  steps TEXT NOT NULL,
  repeat INTEGER DEFAULT 0
);  