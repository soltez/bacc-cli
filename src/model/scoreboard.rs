use bacc::BaccaratScoreboard;

pub struct ScoreboardCache {
    bead_plate: Vec<u8>,
    big_road: Vec<(u8, u8)>,
    derived: [Vec<(u8, u8)>; 3],
}

impl ScoreboardCache {
    pub fn new() -> Self {
        ScoreboardCache {
            bead_plate: Vec::new(),
            big_road: Vec::new(),
            derived: [Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn bead_plate(&self) -> &[u8] {
        &self.bead_plate
    }

    pub fn big_road(&self) -> &[(u8, u8)] {
        &self.big_road
    }

    pub fn derived_road(&self, idx: usize) -> &[(u8, u8)] {
        &self.derived[idx]
    }

    pub fn clear(&mut self) {
        self.bead_plate.clear();
        self.big_road.clear();
        for d in &mut self.derived {
            d.clear();
        }
    }

    pub fn update(&mut self, sb: &BaccaratScoreboard) {
        let bead = (sb.bead_plate().iter_u32_digits().next().unwrap_or(0) & 0xFF) as u8;
        self.bead_plate.push(bead);

        if bead & 0x03 == 3 {
            return;
        }

        let limb = sb.big_road().iter_u32_digits().next().unwrap_or(0);
        let count = (limb & 0xFF) as u8;
        let marker = ((limb >> 8) & 0x03) as u8;
        if count == 1 {
            self.big_road.push((marker, count));
        } else if let Some(last) = self.big_road.last_mut() {
            last.1 = count;
        }

        for (i, road) in sb.derived_roads().iter().enumerate() {
            let byte = (road.iter_u32_digits().next().unwrap_or(0) & 0xFF) as u8;
            let count = (byte & 0xFE) >> 1;
            let marker = if byte & 0x01 == 1 { 2 } else { 1 };
            if count == 1 {
                self.derived[i].push((count, marker));
            } else if let Some(last) = self.derived[i].last_mut() {
                last.0 = count;
            }
        }
    }
}
