export function formatName(name: string) {
    return name.replace(/([A-Z])/g, " $1").replace(/^./, str => str.toUpperCase());
}
