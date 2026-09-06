import * as THREE from "three";
import type { Bounds, TreeOutput } from "./core";

function box(bounds: Bounds | null): THREE.Box3 {
  return bounds ? new THREE.Box3(new THREE.Vector3(...bounds.min), new THREE.Vector3(...bounds.max)) : new THREE.Box3();
}
/** Geometry takes ownership of the JS copies. Materials remain caller-owned.
 * Native bounds describe immutable generated buffers; rebuild after editing them. */
export function materializeTree(output: TreeOutput, materials: { surface: THREE.Material; element: THREE.Material }): THREE.Group {
  const group = new THREE.Group(); group.name = "grower-tree";
  try {
    if (output.surface) {
      const s = output.surface;
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.BufferAttribute(s.positions, 3));
      geometry.setAttribute("normal", new THREE.BufferAttribute(s.normals, 3));
      geometry.setIndex(new THREE.BufferAttribute(s.indices, 1));
      geometry.boundingBox = box(s.bounds);
      geometry.boundingSphere = geometry.boundingBox.getBoundingSphere(new THREE.Sphere());
      const mesh = new THREE.Mesh(geometry, materials.surface); mesh.name = "grower-trunk"; group.add(mesh);
    }
    if (output.foliage?.matrices.length) {
      const f = output.foliage;
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.BufferAttribute(f.positions, 3));
      geometry.setIndex(new THREE.BufferAttribute(f.indices, 1));
      geometry.computeVertexNormals();
      geometry.computeBoundingBox();
      const mesh = new THREE.InstancedMesh(geometry, materials.element, 0);
      mesh.count = f.matrices.length / 16;
      mesh.instanceMatrix = new THREE.InstancedBufferAttribute(f.matrices, 16);
      recomputeInstanceBounds(mesh);
      mesh.userData.anatomy = f.anatomy;
      mesh.name = "grower-canopy"; group.add(mesh);
    }
    return group;
  } catch (error) { disposeTreeGeometry(group); throw error; }
}
export function disposeTreeGeometry(group: THREE.Object3D): void {
  group.traverse(object => {
    if (object instanceof THREE.Mesh) { object.geometry.dispose(); if (object instanceof THREE.InstancedMesh) object.dispose(); }
  });
}

/** Measure transformed vertices, including connectors, rather than the prototype
 * box. Call again after changing matrices or the displayed instance count. */
export function recomputeInstanceBounds(mesh: THREE.InstancedMesh): void {
  const bounds = new THREE.Box3();
  const matrix = new THREE.Matrix4();
  const vertex = new THREE.Vector3();
  const positions = mesh.geometry.getAttribute("position");
  for (let i = 0; i < mesh.count; i++) {
    mesh.getMatrixAt(i, matrix);
    for (let v = 0; v < positions.count; v++) {
      bounds.expandByPoint(vertex.fromBufferAttribute(positions, v).applyMatrix4(matrix));
    }
  }
  mesh.boundingBox = bounds;
  mesh.boundingSphere = bounds.getBoundingSphere(new THREE.Sphere());
  mesh.userData.nativeBounds = true;
}
