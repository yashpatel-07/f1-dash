export interface DriverStanding {
  position: string;
  points: string;
  wins: string;
  driverName: string;
  teamName: string;
  differenceToLeader?: string;
}

export interface RaceStatus {
  season: string;
  round: string;
  raceName: string;
  date: string;
  time: string;
  firstPractice: Session | null;
  secondPractice: Session | null;
  thirdPractice: Session | null;
  qualifying: Session | null;
  sprint: Session | null;
  sprintQualifying: Session | null;
}

interface Session {
  date: string;
  time: string;
}

export interface ConstructorStatus {
  position: string;
  points: string;
  wins: string;
  teamName: string;
}
